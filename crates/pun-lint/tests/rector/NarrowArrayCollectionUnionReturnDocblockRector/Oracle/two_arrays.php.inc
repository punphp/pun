<?php

use Doctrine\Common\Collections\ArrayCollection;

abstract class SomeClass
{
    /**
     * @return \DateTime[]|int[]
     */
    public function getItems()
    {
        return [];
    }
}
