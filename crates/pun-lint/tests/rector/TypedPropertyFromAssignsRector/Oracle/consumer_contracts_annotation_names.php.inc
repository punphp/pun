<?php

final class ConsumerAnnotationNames
{
    /** @\Doctrine\ORM\Mapping\Column(type="integer") */
    private $mapped;

    /** @Column(type="integer") */
    private $ordinary;

    public function __construct(int $value)
    {
        $this->mapped = $value;
        $this->ordinary = $value;
    }
}
-----
<?php

final class ConsumerAnnotationNames
{
    /** @\Doctrine\ORM\Mapping\Column(type="integer") */
    private $mapped;

    /** @Column(type="integer") */
    private int $ordinary;

    public function __construct(int $value)
    {
        $this->mapped = $value;
        $this->ordinary = $value;
    }
}
