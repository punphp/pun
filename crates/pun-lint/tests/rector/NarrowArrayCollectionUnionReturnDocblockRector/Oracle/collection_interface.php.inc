<?php

use Doctrine\Common\Collections\Collection;

final class CollectionInterface
{
    private $items;

    /**
     * @return Collection|string[]
     */
    public function getItems()
    {
        return $this->items;
    }
}
-----
<?php

use Doctrine\Common\Collections\Collection;

final class CollectionInterface
{
    private $items;

    /**
     * @return Collection<int, string>
     */
    public function getItems()
    {
        return $this->items;
    }
}
