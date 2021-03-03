<?php

namespace App\Guards;

interface Guard
{
    /**
     * @throws AccessDeniedException when guard denies access
     */
    public function check(): void;
}
