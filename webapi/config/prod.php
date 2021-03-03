<?php

namespace _
{
    error_reporting(E_ALL ^ E_USER_DEPRECATED);
    ini_set('display_errors', '0');

    // Bring all errors
    use App\ErrorHandler;
    use League\BooBoo\BooBoo;
    use League\BooBoo\Formatter\NullFormatter;
    use League\BooBoo\Handler\LogHandler;
    use Monolog\Handler\StreamHandler;
    use Monolog\Logger;

    $log = new Logger('name');
    $log->pushHandler(new StreamHandler('grrr.log', Logger::WARNING));

    $booboo = new BooBoo([new NullFormatter]);
    $booboo->pushHandler(new LogHandler($log));
    $booboo->pushHandler(new ErrorHandler);
    $booboo->register(); // Registers the handlers
}

