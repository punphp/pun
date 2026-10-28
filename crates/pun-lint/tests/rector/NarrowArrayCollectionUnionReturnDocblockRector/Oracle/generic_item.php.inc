<?php

use Doctrine\Common\Collections\ArrayCollection;

abstract class SomeClass
{
    /**
     * @return array<int>[]|ArrayCollection
     */
    public function getItems()
    {
        return [];
    }
}
