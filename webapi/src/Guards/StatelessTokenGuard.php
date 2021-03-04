<?php

namespace App\Guards;

use App\Core\Env;
use Siler\Http\Request;

class StatelessTokenGuard implements Guard
{
    private string $tokenPrefix;
    private int $tokenPrefixLen;
    private int $tokenMaxLen;
    private int $tokenMinLen;

    public function __construct()
    {
        // Note THE TRAILING SPACE, required as fuck
        $this->tokenPrefix = 'Bearer' . ' ';
        $this->tokenPrefixLen = strlen($this->tokenPrefix);

        // Sample decoded token
        // <guild ID>,<valid until>,<checksum>
        //    <6>   <1>   <10>    <1>  <40>    = 58 - minimal length of valid token
        // <6> = length of earliest possible snowflake
        // <10> = length of earliest possible timestamp
        // <40> = length of sha1
        // <1> = , (comma length)
        $this->tokenMaxLen = 72; // 20 (max discord snowflake len) + 10 (int32/timestamp max len) + 40 (sha1) + 2 (commas)
        $this->tokenMinLen = 58;
    }

    public function check(): void
    {
        $auth = Request\header('Authorization');

        if (! $auth) {
            throw new AccessDeniedException("missing auth header");
        }

        if (1 !== preg_match("#^{$this->tokenPrefix}[a-zA-Z0-9+/=]+$#", $auth)) {
            throw new AccessDeniedException("invalid auth header");
        }

        $token = substr($auth, $this->tokenPrefixLen);

        $decoded = base64_decode($token, true);

        if (! $decoded) {
            throw new AccessDeniedException("invalid token b64");
        }

        $decodedLen = strlen($decoded);
        if ($decodedLen < $this->tokenMinLen || $decodedLen > $this->tokenMaxLen) {
            throw new AccessDeniedException("token too long/short");
        }

        $pattern = "#^(?<guild>[1-9]\d{5,19}),(?<timestamp>[1-9]\d{9}),(?<checksum>[a-f0-9]{40})$#";
        if (1 !== preg_match($pattern, $decoded, $matches)) {
            throw new AccessDeniedException("invalid token pattern");
        }

        [
            'guild' => $guildId,
            'timestamp' => $validUntil,
            'checksum' => $checksum,
        ] = $matches;

        if (time() >= ($validUntil - Env::getTokenTime())) {
            throw new AccessDeniedException("token expired");
        }

        $givenHash = hash_hmac('sha1', "{$guildId},{$validUntil}", Env::getSecret());

        if (! hash_equals($checksum, $givenHash)) {
            throw new AccessDeniedException("invalid hmac");
        }
    }
}
