<?php declare(strict_types=1);

namespace PhpParser;

use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Expr\BinaryOp\Concat;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar\String_;
use PhpParser\Node\Stmt\Use_;

class BuilderFactory {
    /**
     * Creates an attribute node.
     *
     * @param string|Name $name Name of the attribute
     * @param array $args Attribute named arguments
     */
    public function attribute($name, array $args = []): Node\Attribute {}

    /**
     * Creates a namespace builder.
     *
     * @param null|string|Node\Name $name Name of the namespace
     *
     * @return Builder\Namespace_ The created namespace builder
     */
    public function namespace($name): Builder\Namespace_ {}

    /**
     * Creates a class builder.
     *
     * @param string $name Name of the class
     *
     * @return Builder\Class_ The created class builder
     */
    public function class(string $name): Builder\Class_ {}

    /**
     * Creates an interface builder.
     *
     * @param string $name Name of the interface
     *
     * @return Builder\Interface_ The created interface builder
     */
    public function interface(string $name): Builder\Interface_ {}

    /**
     * Creates a trait builder.
     *
     * @param string $name Name of the trait
     *
     * @return Builder\Trait_ The created trait builder
     */
    public function trait(string $name): Builder\Trait_ {}

    /**
     * Creates an enum builder.
     *
     * @param string $name Name of the enum
     *
     * @return Builder\Enum_ The created enum builder
     */
    public function enum(string $name): Builder\Enum_ {}

    /**
     * Creates a trait use builder.
     *
     * @param Node\Name|string ...$traits Trait names
     *
     * @return Builder\TraitUse The created trait use builder
     */
    public function useTrait(...$traits): Builder\TraitUse {}

    /**
     * Creates a trait use adaptation builder.
     *
     * @param Node\Name|string|null $trait Trait name
     * @param Node\Identifier|string $method Method name
     *
     * @return Builder\TraitUseAdaptation The created trait use adaptation builder
     */
    public function traitUseAdaptation($trait, $method = null): Builder\TraitUseAdaptation {}

    /**
     * Creates a method builder.
     *
     * @param string $name Name of the method
     *
     * @return Builder\Method The created method builder
     */
    public function method(string $name): Builder\Method {}

    /**
     * Creates a parameter builder.
     *
     * @param string $name Name of the parameter
     *
     * @return Builder\Param The created parameter builder
     */
    public function param(string $name): Builder\Param {}

    /**
     * Creates a property builder.
     *
     * @param string $name Name of the property
     *
     * @return Builder\Property The created property builder
     */
    public function property(string $name): Builder\Property {}

    /**
     * Creates a function builder.
     *
     * @param string $name Name of the function
     *
     * @return Builder\Function_ The created function builder
     */
    public function function(string $name): Builder\Function_ {}

    /**
     * Creates a namespace/class use builder.
     *
     * @param Node\Name|string $name Name of the entity (namespace or class) to alias
     *
     * @return Builder\Use_ The created use builder
     */
    public function use($name): Builder\Use_ {}

    /**
     * Creates a function use builder.
     *
     * @param Node\Name|string $name Name of the function to alias
     *
     * @return Builder\Use_ The created use function builder
     */
    public function useFunction($name): Builder\Use_ {}

    /**
     * Creates a constant use builder.
     *
     * @param Node\Name|string $name Name of the const to alias
     *
     * @return Builder\Use_ The created use const builder
     */
    public function useConst($name): Builder\Use_ {}

    /**
     * Creates a class constant builder.
     *
     * @param string|Identifier $name Name
     * @param Node\Expr|bool|null|int|float|string|array $value Value
     *
     * @return Builder\ClassConst The created use const builder
     */
    public function classConst($name, $value): Builder\ClassConst {}

    /**
     * Creates an enum case builder.
     *
     * @param string|Identifier $name Name
     *
     * @return Builder\EnumCase The created use const builder
     */
    public function enumCase($name): Builder\EnumCase {}

    /**
     * Creates node a for a literal value.
     *
     * @param Expr|bool|null|int|float|string|array|\UnitEnum $value $value
     */
    public function val($value): Expr {}

    /**
     * Creates variable node.
     *
     * @param string|Expr $name Name
     */
    public function var($name): Expr\Variable {}

    /**
     * Normalizes an argument list.
     *
     * Creates Arg nodes for all arguments and converts literal values to expressions.
     *
     * @param array $args List of arguments to normalize
     *
     * @return list<Arg>
     */
    public function args(array $args): array {}

    /**
     * Creates a function call node.
     *
     * @param string|Name|Expr $name Function name
     * @param array $args Function arguments
     */
    public function funcCall($name, array $args = []): Expr\FuncCall {}

    /**
     * Creates a method call node.
     *
     * @param Expr $var Variable the method is called on
     * @param string|Identifier|Expr $name Method name
     * @param array $args Method arguments
     */
    public function methodCall(Expr $var, $name, array $args = []): Expr\MethodCall {}

    /**
     * Creates a static method call node.
     *
     * @param string|Name|Expr $class Class name
     * @param string|Identifier|Expr $name Method name
     * @param array $args Method arguments
     */
    public function staticCall($class, $name, array $args = []): Expr\StaticCall {}

    /**
     * Creates an object creation node.
     *
     * @param string|Name|Expr $class Class name
     * @param array $args Constructor arguments
     */
    public function new($class, array $args = []): Expr\New_ {}

    /**
     * Creates a constant fetch node.
     *
     * @param string|Name $name Constant name
     */
    public function constFetch($name): Expr\ConstFetch {}

    /**
     * Creates a property fetch node.
     *
     * @param Expr $var Variable holding object
     * @param string|Identifier|Expr $name Property name
     */
    public function propertyFetch(Expr $var, $name): Expr\PropertyFetch {}

    /**
     * Creates a class constant fetch node.
     *
     * @param string|Name|Expr $class Class name
     * @param string|Identifier|Expr $name Constant name
     */
    public function classConstFetch($class, $name): Expr\ClassConstFetch {}

    /**
     * Creates nested Concat nodes from a list of expressions.
     *
     * @param Expr|string ...$exprs Expressions or literal strings
     */
    public function concat(...$exprs): Concat {}

    /**
     * @param string|Expr $expr
     */
    private function normalizeStringExpr($expr): Expr {}
}
