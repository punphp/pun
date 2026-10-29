<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;

final class SkipDocWithoutReturn
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * Gets the repository.
     */
    public function getRepository()
    {
        return $this->em->getRepository(Event::class);
    }
}
