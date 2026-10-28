<?php

use Doctrine\Common\Collections\ArrayCollection;

abstract class SomeClass
{
    /**
     * @return array<
     *     int
     * >
     */
    public function getItems()
    {
        return [];
    }
}
