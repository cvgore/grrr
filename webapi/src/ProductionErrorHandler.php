<?php

namespace App;

use Siler\Http\Response;
use Whoops\Handler\Handler;

class ProductionErrorHandler extends Handler
{
    public function handle(): ?int
    {
        Response\json(['error' => 'internal server error']);

        return Handler::DONE;
    }
}
