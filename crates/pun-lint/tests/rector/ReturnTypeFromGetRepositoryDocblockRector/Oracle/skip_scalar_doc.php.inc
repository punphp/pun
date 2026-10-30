<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;

final class SkipScalarDoc
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return string
     */
    public function getRepository()
    {
        return $this->em->getRepository(Event::class);
    }
}
