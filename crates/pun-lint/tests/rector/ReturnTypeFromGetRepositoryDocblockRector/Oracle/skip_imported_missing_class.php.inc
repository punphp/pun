<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromGetRepositoryDocblockRector\Oracle;

use App\Missing\MissingRepository;
use Doctrine\ORM\EntityManagerInterface;

final class SkipImportedMissingClass
{
    public function __construct(private EntityManagerInterface $em)
    {
    }

    /**
     * @return MissingRepository
     */
    public function getRepository()
    {
        return $this->em->getRepository(Event::class);
    }
}
