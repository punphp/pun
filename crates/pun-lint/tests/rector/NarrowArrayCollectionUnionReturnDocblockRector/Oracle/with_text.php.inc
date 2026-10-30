<?php

use Doctrine\Common\Collections\ArrayCollection;

final class WithText
{
    private $items;

    /**
     * Gives the items.
     *
     * @return \DateTime[]|ArrayCollection the items
     */
    public function getItems()
    {
        return $this->items;
    }
}
-----
<?php

use Doctrine\Common\Collections\ArrayCollection;

final class WithText
{
    private $items;

    /**
     * Gives the items.
     *
     * @return ArrayCollection<int, \DateTime> the items
     */
    public function getItems()
    {
        return $this->items;
    }
}
