<?php

namespace App\Controllers;

//use Siler\Functional as L;
use Siler\Http\Response;

class DefaultController
{
    public function index(): void
    {
        Response\json(['app' =>'Grrr web API']);
    }
}
