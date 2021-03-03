<?php declare(strict_types=1);

define('_GRR_WEB', 1);

require_once __DIR__ . '/../vendor/autoload.php';

use App\Controllers\ConfigureController;
use App\Controllers\DefaultController;
use App\Core\Env;
use Siler\Functional as L;
use Siler\Route;
use function App\Functional\lcallm;

Env::start();
Env::loadConfig();

Route\get('/{guildId}/configure', lcallm([ConfigureController::class, 'get']));

Route\get('/', lcallm([DefaultController::class, 'index']));

if (! Route\did_match()) {
    L\call([new DefaultController, 'index']);
}
