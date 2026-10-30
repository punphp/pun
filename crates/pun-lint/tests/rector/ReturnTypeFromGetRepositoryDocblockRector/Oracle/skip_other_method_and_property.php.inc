<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use Doctrine\ORM\EntityManagerInterface;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Source\EventRepository;

final class SkipOtherMethodAndProperty
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return EventRepository
     */
    public function find()
    {
        return $this->em->find(Event::class);
    }

    /**
     * @return EventRepository
     */
    public function property()
    {
        return $this->em;
    }

    /**
     * @return EventRepository
     */
    public function unassigned()
    {
        return $repo;
    }
}
