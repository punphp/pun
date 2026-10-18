<?php

final class CommentOnlyGetter
{
    private $some;

    public function run()
    {
        return $this->getSome() + 5;
    }

    private function getSome()
    {
        // nothing yet
    }
}
