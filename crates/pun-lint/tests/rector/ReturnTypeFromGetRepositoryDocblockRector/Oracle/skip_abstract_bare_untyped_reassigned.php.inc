<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository;

abstract class SkipAbstractBareUntypedReassigned
{
    private $untyped;

    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return EventRepository
     */
    abstract public function abstractRepository();

    /**
     * @return EventRepository
     */
    public function bare()
    {
        return;
    }

    /**
     * @return EventRepository
     */
    public function untyped()
    {
        return $this->untyped->getRepository(Event::class);
    }
}

final class OtherVariableAssigned
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return EventRepository
     */
    public function getRepository()
    {
        $other = 1;
        $repo = $this->em->getRepository(Event::class);

        return $repo;
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository;

abstract class SkipAbstractBareUntypedReassigned
{
    private $untyped;

    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return EventRepository
     */
    abstract public function abstractRepository();

    /**
     * @return EventRepository
     */
    public function bare()
    {
        return;
    }

    /**
     * @return EventRepository
     */
    public function untyped()
    {
        return $this->untyped->getRepository(Event::class);
    }
}

final class OtherVariableAssigned
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    public function getRepository(): \Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository
    {
        $other = 1;
        $repo = $this->em->getRepository(Event::class);

        return $repo;
    }
}
