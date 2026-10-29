<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository;

final class SkipTwoReturns
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return EventRepository
     */
    public function getRepository(bool $flag)
    {
        if ($flag) {
            return $this->em->getRepository(Event::class);
        }

        return $this->em->getRepository(Other::class);
    }
}
