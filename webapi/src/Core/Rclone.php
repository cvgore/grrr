<?php

namespace App\Core\Rclone;

use App\Core\Env;
use function Siler\array_get;

const BACKENDS = [
    'gdrive'
];

function config_key(string $suffix = null): string {
    return 'grrr_' . $suffix;
}

function supported_backends(): array {
    return BACKENDS;
}

function config(string $guildId): ?array {
    $config = parse_ini_file(realpath(Env::getRcloneConfigPath()), true, INI_SCANNER_TYPED);

    return array_get($config, config_key($guildId));
}
