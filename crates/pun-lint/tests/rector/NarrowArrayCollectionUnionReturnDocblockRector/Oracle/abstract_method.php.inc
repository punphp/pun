<?php

use Doctrine\Common\Collections\ArrayCollection;

abstract class AbstractMethod
{
    /**
     * @return \DateTime[]|ArrayCollection
     */
    abstract public function getItems();
}
-----
<?php

use Doctrine\Common\Collections\ArrayCollection;

abstract class AbstractMethod
{
    /**
     * @return ArrayCollection<int, \DateTime>
     */
    abstract public function getItems();
}
