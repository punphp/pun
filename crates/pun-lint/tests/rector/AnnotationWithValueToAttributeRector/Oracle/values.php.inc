<?php

namespace App\Tests\Values;

use PHPUnit\Framework\TestCase;

final class ValuesTest extends TestCase
{
    private $property;

    public function testWithoutDoc(): void
    {
    }

    /**
     * @backupStaticAttributes enabled
     */
    public function testInteger(): void
    {
    }

    /**
     * @backupStaticAttributes DISABLED
     */
    public function testFloat(): void
    {
    }

    /**
     * @backupStaticAttributes maybe
     * @Some\Tag value
     */
    public function testString(): void
    {
    }

    /**
     * @backupStaticAttributes unknown
     */
    public function testUnknown(): void
    {
    }

    public function testAnonymous(): void
    {
        new class extends TestCase {
            /**
             * @backupGlobals enabled
             */
            public function testInner(): void
            {
            }
        };
    }
}
-----
<?php

namespace App\Tests\Values;

use PHPUnit\Framework\TestCase;

final class ValuesTest extends TestCase
{
    private $property;

    public function testWithoutDoc(): void
    {
    }

    #[\PHPUnit\Framework\Attributes\BackupStaticProperties(1)]
    public function testInteger(): void
    {
    }

    #[\PHPUnit\Framework\Attributes\BackupStaticProperties(2.5)]
    public function testFloat(): void
    {
    }

    /**
     * @Some\Tag value
     */
    #[\PHPUnit\Framework\Attributes\BackupStaticProperties('yes')]
    public function testString(): void
    {
    }

    #[\PHPUnit\Framework\Attributes\BackupStaticProperties(null)]
    public function testUnknown(): void
    {
    }

    public function testAnonymous(): void
    {
        new class extends TestCase {
            #[\PHPUnit\Framework\Attributes\BackupGlobals(true)]
            public function testInner(): void
            {
            }
        };
    }
}
