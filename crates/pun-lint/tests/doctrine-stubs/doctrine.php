<?php

// The Doctrine classes Rector's fixtures name, as rector-src's test vendor has them: declared, so PHPStan's
// reflection finds them (only their names and hierarchy matter to the property type rules).

namespace Doctrine\ORM {
    interface EntityManagerInterface
    {
    }

    // rector-src's stubs/Doctrine/ORM/EntityRepository.php
    class EntityRepository
    {
    }
}

namespace Doctrine\ORM\Mapping {
    class ClassMetadata
    {
    }
}

namespace Doctrine\Common\Collections {
    interface Collection
    {
    }

    class ArrayCollection implements Collection
    {
    }
}
