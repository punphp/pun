<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository;

final class SkipPrivateMethodMagic
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return EventRepository
     */
    public function __get($name)
    {
        return $this->em->getRepository(Event::class);
    }
}
