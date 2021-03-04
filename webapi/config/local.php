<?php

namespace _
{
    error_reporting(E_ALL);
    ini_set('display_errors', '1');

    use App\ErrorHandler;
    use Whoops\Handler\PrettyPageHandler;
    use Whoops\Run;

    $whoops = new Run;
    $whoops->pushHandler(new PrettyPageHandler);
    $whoops->prependHandler(new ErrorHandler);
    $whoops->register();
}

