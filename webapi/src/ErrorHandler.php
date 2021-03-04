<?php

namespace App;

use App\Guards\AccessDeniedException;
use Siler\Http\Response;
use Throwable;
use Whoops\Handler\Handler;

class ErrorHandler extends Handler
{
    public function handle(): ?int
    {
        $ex = $this->getException();

        if (!$this->handleException($ex)) {
            return Handler::DONE;
        }

        $this->getRun()->sendHttpCode(false);

        return Handler::QUIT;
    }

    /**
     * @param \Throwable $ex
     *
     * @return bool if error was handled, otherwise false
     */
    protected function handleException(Throwable $ex): bool
    {
        if ($ex instanceof AccessDeniedException) {
            Response\json([
                'error' => 'Unauthorized',
            ], 401);

            return true;
        }

        return false;
    }
}
