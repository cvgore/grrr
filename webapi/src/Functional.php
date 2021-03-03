<?php

namespace App\Functional;

use App\Guards\Guard;
use Closure;
use Siler\Functional as L;

/**
 * Lazy call class method
 *
 * @param array $cb
 *
 * @return \Closure
 */
function lcallm(array $cb): Closure
{
    return static fn() => L\call([new $cb[0], $cb[1]], func_get_args());
}

function guard(Guard $guardName): void
{
    L\call([$guardName, 'check']);
}

