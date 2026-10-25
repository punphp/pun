<?php
namespace MethodCallable;

class Ancestor
{
    private function Hidden(): void {}
    protected function ProtectedMethod(): void {}
    public function Required(int $value): void {}
    public static function RequiredStatic(int $value): void {}
}

abstract class Subject extends Ancestor
{
    private function PrivateMethod(): void {}
    private static function PrivateStatic(): void {}
    protected static function ProtectedStatic(): void {}
    abstract protected static function AbstractStatic(): void;
    abstract public function AbstractInstance(): void;

    public function create(): void
    {
        $this->Required(...);
        $this->required(...);
        $this->Missing(...);
        $this->Hidden(...);
        $this->hidden(...);
        $this->PrivateMethod(...);
        $this->AbstractInstance(...);
        self::Required(...);
        self::required(...);
        self::RequiredStatic(...);
        self::requiredstatic(...);
        self::Missing(...);
        self::PrivateStatic(...);
        self::AbstractStatic(...);
        static::AbstractStatic(...);
        self::AbstractInstance(...);
        static::AbstractInstance(...);
        parent::RequiredStatic(...);
        $this::Required(...);
        $this::Missing(...);
        if (method_exists($this, 'Guarded')) {
            $this->Guarded(...);
        }
        if (method_exists(static::class, 'Guarded')) {
            static::Guarded(...);
        }
        $this->Guarded(...);
        static::Guarded(...);
        $name = 'Missing';
        $this->$name(...);
        self::$name(...);
        static function (): void { $this->Missing(...); };
    }
}

class Magic
{
    public function __call(string $name, array $args): mixed {}
    public static function __callStatic(string $name, array $args): mixed {}
    public function create(): void
    {
        $this->Missing(...);
        self::Missing(...);
        $this::Missing(...);
    }
}

/** @method void Annotation(int $value) */
class Annotated
{
    public function create(): void { $this->Annotation(...); }
}

/** @mixin Ancestor */
class MixedIn
{
    public function create(): void { $this->Required(...); self::RequiredStatic(...); }
}

/**
 * @template T of Ancestor
 * @mixin T
 */
class GenericMixin
{
    public function create(): void { $this->Required(...); self::RequiredStatic(...); }
}

/** @phpstan-require-extends Ancestor */
interface RequiredAncestor
{
    public function create(): void;
}

function outside(): void
{
    Subject::RequiredStatic(...);
    Subject::requiredstatic(...);
    Subject::Missing(...);
    Subject::Required(...);
    Subject::PrivateMethod(...);
    Subject::PrivateStatic(...);
    Subject::privatestatic(...);
    Subject::ProtectedStatic(...);
    Subject::AbstractStatic(...);
    Absent::Missing(...);
    if (class_exists(Absent::class)) { Absent::Missing(...); }
    self::Missing(...);
    parent::Missing(...);
    static::Missing(...);
}

class NoParent
{
    public function create(): void { parent::Missing(...); }
}

/**
 * @param class-string<Subject> $class
 * @param class-string $bareClass
 * @param Subject|int $union
 * @param mixed $explicit
 */
function typed(
    Subject $object,
    Ancestor $ancestor,
    Subject $child,
    Magic $magic,
    Annotated $annotation,
    MixedIn $mixin,
    RequiredAncestor $required,
    Absent $unknown,
    int $integer,
    ?Subject $nullable,
    string $string,
    $class,
    $bareClass,
    $union,
    $explicit,
    $implicit,
): void {
    $object->Required(...);
    $object->required(...);
    $object->Missing(...);
    $object->PrivateMethod(...);
    $object->privatemethod(...);
    $ancestor->ProtectedMethod(...);
    $child->Hidden(...);
    $unknown->Missing(...);
    $integer->Missing(...);
    $nullable->Required(...);
    $class->Required(...);
    $string->Required(...);
    $union->Required(...);
    $explicit->Required(...);
    $implicit->Required(...);
    $magic->Missing(...);
    $annotation->Annotation(...);
    $mixin->Required(...);
    $required->Required(...);
    $object::RequiredStatic(...);
    $object::requiredstatic(...);
    $object::Required(...);
    $object::PrivateStatic(...);
    $object::ProtectedStatic(...);
    $object::Missing(...);
    $object::AbstractStatic(...);
    $unknown::Missing(...);
    $integer::Missing(...);
    $nullable::RequiredStatic(...);
    $class::RequiredStatic(...);
    $class::Missing(...);
    $bareClass::Missing(...);
    $string::Missing(...);
    $union::RequiredStatic(...);
    $explicit::RequiredStatic(...);
    $implicit::RequiredStatic(...);
    $magic::Missing(...);
    $mixin::RequiredStatic(...);
    if (method_exists($object, 'Guarded')) { $object->Guarded(...); }
    if (method_exists($class, 'Guarded')) { $class::Guarded(...); }
    $selector = 'Missing';
    $object->$selector(...);
    $object::$selector(...);
}
