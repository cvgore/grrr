<?php

namespace App;

use App\Guards\AccessDeniedException;
use League\BooBoo\Handler\HandlerInterface;
use Siler\Http\Response;

class ErrorHandler implements HandlerInterface
{
    public function handle($e)
    {
        if ($e instanceof AccessDeniedException) {
            Response\json([
                'error' => 'Unauthorized'
            ], 401);
        }
    }
}
