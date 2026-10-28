<?php
namespace DuplicateConstantSelection;

class Ordinary {
    private const CHOICE = 'first';
    /** @var positive-int */
    public const int CHOICE = 29;
    public const GROUP = 47, GROUP = 83;
    public static function inspect(): void { \PHPStan\dumpType(static::CHOICE); }
}
class PrivateLast {
    public const HIDDEN = 'first';
    private const HIDDEN = 31;
    public static function inspect(): void { \PHPStan\dumpType(self::HIDDEN); }
}
class DocumentedLast {
    /** @var string */ public const VALUE = 'first';
    /** @var positive-int */ public const VALUE = 37;
    public static function inspect(): void { \PHPStan\dumpType(static::VALUE); }
}
interface Contract {
    public const VALUE = 41;
    public const VALUE = 43;
}
class ContractUser implements Contract {}

trait Twice {
    private const VALUE = self::FIRST;
    public const VALUE = self::LAST;
    public const GROUP = 13, GROUP = 17;
}
class First {
    use Twice;
    public const FIRST = 11;
    public const LAST = 29;
}
class Second {
    use Twice;
    public const FIRST = 71;
    public const LAST = 83;
}
class Inherited extends First { public const LAST = 97; }
class Reused extends First { use Twice; public const LAST = 101; }
class OwnFirst {
    use Twice;
    public const FIRST = 11;
    public const LAST = 29;
    public const VALUE = 11;
}
class OwnLast {
    use Twice;
    public const FIRST = 11;
    public const LAST = 29;
    public const VALUE = 29;
}
trait TypedTwice {
    public const string VALUE = 'first';
    public const int VALUE = 53;
}
class TypedConsumer { use TypedTwice; }

\PHPStan\dumpType(Ordinary::CHOICE);
\PHPStan\dumpType(Ordinary::GROUP);
\PHPStan\dumpType(PrivateLast::HIDDEN);
\PHPStan\dumpType(DocumentedLast::VALUE);
\PHPStan\dumpType(ContractUser::VALUE);
\PHPStan\dumpType(First::VALUE);
\PHPStan\dumpType(Second::VALUE);
\PHPStan\dumpType(Inherited::VALUE);
\PHPStan\dumpType(Reused::VALUE);
\PHPStan\dumpType(OwnFirst::VALUE);
\PHPStan\dumpType(OwnLast::VALUE);
\PHPStan\dumpType(TypedConsumer::VALUE);
\PHPStan\dumpType(First::GROUP);
