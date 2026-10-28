<?php

use Doctrine\Common\Collections\ArrayCollection;

abstract class SomeClass
{
    /**
     * @return \DateTime[]|ArrayCollection|null
     */
    public function getItems()
    {
        return [];
    }
}
