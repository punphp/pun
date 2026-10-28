<?php

namespace App;

use App\Repository\ProductRepository as Products;
use PHPUnit\Framework\TestCase;

final class ProductRepositoryExists {}

final class RepositoryTest extends TestCase
{
    /**
     * @var ProductRepositoryExists
     */
    private $products;
    /** @var ProductRepositoryExists The repository */
    private $described;
    /** @var Products */
    private $aliased;
    /** @var Missing */
    private $missing;
    /** @var self */
    private $self;
    /** @var ProductRepositoryExists|null */
    private $union;
    private $noDoc;
    /** @var ProductRepositoryExists */
    private static $static;
    /** @var ProductRepositoryExists */
    protected $protected;
    /** @var ProductRepositoryExists */
    private $notRepository;
    /** @var ProductRepositoryExists */
    private $nested;

    protected function setUp(): void
    {
        $this->products = $this->em->getRepository(ProductRepositoryExists::class);
        $this->described = $this->em->getRepository('x');
        $this->aliased = $this->em->getRepository('x');
        $this->missing = $this->em->getRepository('x');
        $this->self = $this->em->getRepository('x');
        $this->union = $this->em->getRepository('x');
        $this->noDoc = $this->em->getRepository('x');
        self::$static = $this->em->getRepository('x');
        $this->protected = $this->em->getRepository('x');
        $this->notRepository = $this->em->find('x');
        if (true) {
            $this->nested = $this->em->getRepository('x');
        }
    }
}

final class NotATest
{
    /** @var ProductRepositoryExists */
    private $products;

    protected function setUp(): void
    {
        $this->products = $this->em->getRepository('x');
    }
}
-----
<?php

namespace App;

use App\Repository\ProductRepository as Products;
use PHPUnit\Framework\TestCase;

final class ProductRepositoryExists {}

final class RepositoryTest extends TestCase
{
    private \App\ProductRepositoryExists $products;
    private \App\ProductRepositoryExists $described;
    /** @var Products */
    private $aliased;
    /** @var Missing */
    private $missing;
    /** @var self */
    private $self;
    /** @var ProductRepositoryExists|null */
    private $union;
    private $noDoc;
    /** @var ProductRepositoryExists */
    private static $static;
    /** @var ProductRepositoryExists */
    protected $protected;
    /** @var ProductRepositoryExists */
    private $notRepository;
    private \App\ProductRepositoryExists $nested;

    protected function setUp(): void
    {
        $this->products = $this->em->getRepository(ProductRepositoryExists::class);
        $this->described = $this->em->getRepository('x');
        $this->aliased = $this->em->getRepository('x');
        $this->missing = $this->em->getRepository('x');
        $this->self = $this->em->getRepository('x');
        $this->union = $this->em->getRepository('x');
        $this->noDoc = $this->em->getRepository('x');
        self::$static = $this->em->getRepository('x');
        $this->protected = $this->em->getRepository('x');
        $this->notRepository = $this->em->find('x');
        if (true) {
            $this->nested = $this->em->getRepository('x');
        }
    }
}

final class NotATest
{
    /** @var ProductRepositoryExists */
    private $products;

    protected function setUp(): void
    {
        $this->products = $this->em->getRepository('x');
    }
}
