<?php
class SingleCallReturnExtension implements PHPStan\Type\DynamicMethodReturnTypeExtension {
    public function getClass(): string { return 'SingleCallTargets'; }
    public function isMethodSupported(PHPStan\Reflection\MethodReflection $methodReflection): bool {
        return in_array($methodReflection->getName(), ['foo', 'bar'], true);
    }
    public function getTypeFromMethodCall(PHPStan\Reflection\MethodReflection $methodReflection, PhpParser\Node\Expr\MethodCall $methodCall, PHPStan\Analyser\Scope $scope): ?PHPStan\Type\Type {
        $value = $scope->getType($methodCall->getArgs()[0]->value);
        if ($methodReflection->getName() === 'foo') return $value;
        $builder = PHPStan\Type\Constant\ConstantArrayTypeBuilder::createEmpty();
        $builder->setOffsetValueType(null, $value);
        return $builder->getArray();
    }
}
