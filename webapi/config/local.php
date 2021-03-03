<?php

namespace _
{

    // Bring all errors
    use App\ErrorHandler;
    use League\BooBoo\BooBoo;
    use League\BooBoo\Formatter\HtmlFormatter;
    use League\BooBoo\Formatter\HtmlTableFormatter;
    use League\BooBoo\Formatter\JsonFormatter;

    error_reporting(E_ALL);
    ini_set('display_errors', '1');

    $booboo = new BooBoo([new HtmlTableFormatter]);
//    $booboo->pushHandler(new ErrorHandler);
    $booboo->register(); // Registers the handlers
}

