<?php

namespace App;

use Doctrine\ORM\EntityRepository;

final class Product {}

/**
 * @extends EntityRepository<Product>
 */
class ProductRepository extends EntityRepository
{
    public function one($id)
    {
        return $this->createQueryBuilder('p')->getQuery()->getOneOrNullResult();
    }

    /**
     * Finds them.
     */
    public function many(array $criteria)
    {
        return $this->findBy($criteria);
    }

    public function both($id)
    {
        $this->findOneBy(['id' => $id]);
        return $this->findBy([]);
    }

    public function inClosure()
    {
        return fn () => $this->findOneBy([]);
    }

    public function typed($id): ?Product
    {
        return $this->findOneBy(['id' => $id]);
    }

    protected function hidden($id)
    {
        return $this->findOneBy(['id' => $id]);
    }

    public static function make()
    {
        return null;
    }

    public function other()
    {
        return $this->count([]);
    }

    public function staticCall()
    {
        return self::findBy([]);
    }
}

/**
 * @extends \Doctrine\ORM\EntityRepository<Product>
 */
class QualifiedRepository extends EntityRepository
{
    public function one($id)
    {
        return $this->findOneBy(['id' => $id]);
    }
}

/**
 * @extends EntityRepository<\App\Product>
 */
class FullyQualifiedEntity extends EntityRepository
{
    public function one($id)
    {
        return $this->findOneBy(['id' => $id]);
    }
}

/**
 * @extends EntityRepository<Product>
 */
class NotARepository
{
    public function one($id)
    {
        return $this->findOneBy(['id' => $id]);
    }
}
-----
<?php

namespace App;

use Doctrine\ORM\EntityRepository;

final class Product {}

/**
 * @extends EntityRepository<Product>
 */
class ProductRepository extends EntityRepository
{
    public function one($id): ?Product
    {
        return $this->createQueryBuilder('p')->getQuery()->getOneOrNullResult();
    }

    /**
     * Finds them.
     * @return Product[]
     */
    public function many(array $criteria): array
    {
        return $this->findBy($criteria);
    }

    /**
     * @return Product[]
     */
    public function both($id): array
    {
        $this->findOneBy(['id' => $id]);
        return $this->findBy([]);
    }

    public function inClosure(): ?Product
    {
        return fn () => $this->findOneBy([]);
    }

    public function typed($id): ?Product
    {
        return $this->findOneBy(['id' => $id]);
    }

    protected function hidden($id)
    {
        return $this->findOneBy(['id' => $id]);
    }

    public static function make()
    {
        return null;
    }

    public function other()
    {
        return $this->count([]);
    }

    public function staticCall()
    {
        return self::findBy([]);
    }
}

/**
 * @extends \Doctrine\ORM\EntityRepository<Product>
 */
class QualifiedRepository extends EntityRepository
{
    public function one($id)
    {
        return $this->findOneBy(['id' => $id]);
    }
}

/**
 * @extends EntityRepository<\App\Product>
 */
class FullyQualifiedEntity extends EntityRepository
{
    public function one($id): ?\App\Product
    {
        return $this->findOneBy(['id' => $id]);
    }
}

/**
 * @extends EntityRepository<Product>
 */
class NotARepository
{
    public function one($id)
    {
        return $this->findOneBy(['id' => $id]);
    }
}
