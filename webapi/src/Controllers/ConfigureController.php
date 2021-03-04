<?php

namespace App\Controllers;

//use Siler\Functional as L;
use App\Core\Rclone;
use App\Guards\StatelessTokenGuard;
use Siler\Http\Response;
use function App\Functional\guard;

class ConfigureController
{
    public function get(): void
    {
        guard(new StatelessTokenGuard);

        $config = Rclone\config();

        Response\json(['test']);
    }
}
