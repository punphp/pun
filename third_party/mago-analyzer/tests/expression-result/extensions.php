<?php
class ResultFunctionProvider implements PHPStan\Type\DynamicFunctionReturnTypeExtension {
    public function isFunctionSupported(PHPStan\Reflection\FunctionReflection $functionReflection): bool {
        return $functionReflection->getName() === 'resultProvider';
    }
    public function getTypeFromFunctionCall(PHPStan\Reflection\FunctionReflection $functionReflection, PhpParser\Node\Expr\FuncCall $functionCall, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        return new PHPStan\Type\VoidType();
    }
}
class ResultMethodProvider implements PHPStan\Type\DynamicMethodReturnTypeExtension {
    public function getClass(): string { return 'ResultSink'; }
    public function isMethodSupported(PHPStan\Reflection\MethodReflection $methodReflection): bool {
        return $methodReflection->getName() === 'fromProvider';
    }
    public function getTypeFromMethodCall(PHPStan\Reflection\MethodReflection $methodReflection, PhpParser\Node\Expr\MethodCall $methodCall, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        return new PHPStan\Type\VoidType();
    }
}
class ResultExpressionOverride implements PHPStan\Type\ExpressionTypeResolverExtension {
    public function getType(PhpParser\Node\Expr $expr, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        if (!$expr instanceof PhpParser\Node\Expr\FuncCall || !$expr->name instanceof PhpParser\Node\Name) return null;
        return match ($expr->name->toString()) {
            'resultEager' => new PHPStan\Type\VoidType(),
            'resultOverride' => new PHPStan\Type\IntegerType(),
            default => null,
        };
    }
}
