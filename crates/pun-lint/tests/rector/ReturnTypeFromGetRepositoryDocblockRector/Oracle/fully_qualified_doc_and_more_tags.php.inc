<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;

final class FullyQualifiedDocAndMoreTags
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * Gets the repository.
     *
     * @return \Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository
     */
    public function getRepository()
    {
        return $this->em->getRepository(Event::class);
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;

final class FullyQualifiedDocAndMoreTags
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * Gets the repository.
     */
    public function getRepository(): \Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository
    {
        return $this->em->getRepository(Event::class);
    }
}
