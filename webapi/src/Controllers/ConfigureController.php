<?php

namespace App\Controllers;

//use Siler\Functional as L;
use App\Guards\StatelessTokenGuard;
use Siler\Http\Response;
use function App\Functional\guard;

class ConfigureController
{
    public function get(array $args): void
    {
        guard(new StatelessTokenGuard);

        Response\json(['test']);
    }
}
