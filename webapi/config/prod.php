<?php

namespace _
{
    error_reporting(E_ALL ^ E_USER_DEPRECATED);
    ini_set('display_errors', '0');

    use App\ErrorHandler;
    use App\ProductionErrorHandler;
    use Monolog\Handler\StreamHandler;
    use Monolog\Logger;
    use Whoops\Handler\PlainTextHandler;
    use Whoops\Run;

    $log = new Logger('Grrr');
    $log->pushHandler(new StreamHandler('grrr.log', Logger::WARNING));

    $whoops = new Run;
    $logHandler = new PlainTextHandler($log);
    $logHandler->loggerOnly(true);
    $whoops->pushHandler($logHandler);
    $whoops->pushHandler(new ProductionErrorHandler);
    $whoops->prependHandler(new ErrorHandler);
    $whoops->register();
}

