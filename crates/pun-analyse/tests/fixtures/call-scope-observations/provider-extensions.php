<?php
// Independent oracle counterparts of the host providers and semantic hooks.
// Their value contract is identity; the fixture's declared return type is mixed.
class PricingFunctionExtension implements PHPStan\Type\DynamicFunctionReturnTypeExtension {
    public function isFunctionSupported(PHPStan\Reflection\FunctionReflection $functionReflection): bool {
        return in_array($functionReflection->getName(), ['pricingFunction', 'pricingHook', 'pricingExpression'], true);
    }
    public function getTypeFromFunctionCall(PHPStan\Reflection\FunctionReflection $functionReflection, PhpParser\Node\Expr\FuncCall $functionCall, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        return $scope->getType($functionCall->getArgs()[0]->value);
    }
}
class PricingMethodExtension implements PHPStan\Type\DynamicMethodReturnTypeExtension {
    public function getClass(): string { return 'PricingWorker'; }
    public function isMethodSupported(PHPStan\Reflection\MethodReflection $methodReflection): bool {
        return in_array($methodReflection->getName(), ['fromProvider', 'fromHook'], true);
    }
    public function getTypeFromMethodCall(PHPStan\Reflection\MethodReflection $methodReflection, PhpParser\Node\Expr\MethodCall $methodCall, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        return $scope->getType($methodCall->getArgs()[0]->value);
    }
}
class PricingStaticExtension implements PHPStan\Type\DynamicStaticMethodReturnTypeExtension {
    public function getClass(): string { return 'PricingWorker'; }
    public function isStaticMethodSupported(PHPStan\Reflection\MethodReflection $methodReflection): bool {
        return in_array($methodReflection->getName(), ['staticProvider', 'staticHook'], true);
    }
    public function getTypeFromStaticMethodCall(PHPStan\Reflection\MethodReflection $methodReflection, PhpParser\Node\Expr\StaticCall $methodCall, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        return $scope->getType($methodCall->getArgs()[0]->value);
    }
}
