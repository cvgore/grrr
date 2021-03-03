<?php

namespace App\Core;

use Siler\Http\Response;

class Env
{
    private static array $env = [];
    private const ENV_PATH = __DIR__ . '/../../env.php';

    public static function start(): void
    {
        if (! file_exists(self::ENV_PATH)) {
            Response\json(['error' => 'missing env file']);

            die;
        }

        //  Load file with environmental vars
        self::$env = require self::ENV_PATH;
    }

    public static function loadConfig(): void
    {
        // Load base config for environment
        require_once __DIR__ . '/../../config/' . self::getConfig() . '.php';
    }

    public static function getSecret(): string
    {
        return self::$env['secret'];
    }

    public static function getConfig(): string
    {
        return self::$env['config'];
    }

    public static function getTokenTime(): int
    {
        return self::$env['token_time'];
    }
}
