<?php

namespace App;

use Symfony\Component\Serializer\Serializer;

class Base
{
}

final class Holder extends Base
{
    public const TYPE = 'App\Holder';
    public const CLASS_TYPE = Holder::class;

    private Serializer $serializer;

    public function twoStatements($data)
    {
        $format = 'json';
        return $this->serializer->deserialize($data, \stdClass::class, $format);
    }

    public function notReturn($data)
    {
        $this->serializer->deserialize($data, \stdClass::class, 'json');
    }

    public function functionCall($data)
    {
        return deserialize($data, \stdClass::class, 'json');
    }

    public function dynamicName($data, $name)
    {
        return $this->serializer->{$name}($data, \stdClass::class, 'json');
    }

    public function otherMethod($data)
    {
        return $this->serializer->serialize($data, 'json', []);
    }

    public function twoArguments($data)
    {
        return $this->serializer->deserialize($data, \stdClass::class);
    }

    public function spread($data, array $rest)
    {
        return $this->serializer->deserialize($data, \stdClass::class, ...$rest);
    }

    public function named($data)
    {
        return $this->serializer->deserialize($data, \stdClass::class, format: 'json');
    }

    public function untypedSerializer($serializer, $data)
    {
        return $serializer->deserialize($data, \stdClass::class, 'json');
    }

    public function stringType($data)
    {
        return $this->serializer->deserialize($data, 'App\Holder', 'json');
    }

    public function selfClass($data)
    {
        return $this->serializer->deserialize($data, self::class, 'json');
    }

    public function objectClass($data, Holder $holder)
    {
        return $this->serializer->deserialize($data, $holder::class, 'json');
    }

    public function otherConstant($data)
    {
        return $this->serializer->deserialize($data, Holder::TYPE, 'json');
    }

    public function variableType($data, string $type)
    {
        return $this->serializer->deserialize($data, $type, 'json');
    }

    public function staticClass($data)
    {
        return $this->serializer->deserialize($data, static::class, 'json');
    }

    public function parentClass($data)
    {
        return $this->serializer->deserialize($data, parent::class, 'json');
    }

    public function concat($data)
    {
        return $this->serializer->deserialize($data, 'App\\' . 'Holder', 'json');
    }

    public function classConstant($data)
    {
        return $this->serializer->deserialize($data, self::CLASS_TYPE, 'json');
    }

    public function staticConstant($data)
    {
        return $this->serializer->deserialize($data, static::TYPE, 'json');
    }

    public function doubleQuoted($data)
    {
        return $this->serializer->deserialize($data, "App\\Holder", 'json');
    }

    public function intType($data)
    {
        return $this->serializer->deserialize($data, 5, 'json');
    }
}

trait HasParent
{
    private Serializer $serializer;

    public function parentClass($data)
    {
        return $this->serializer->deserialize($data, parent::class, 'json');
    }
}
-----
<?php

namespace App;

use Symfony\Component\Serializer\Serializer;

class Base
{
}

final class Holder extends Base
{
    public const TYPE = 'App\Holder';
    public const CLASS_TYPE = Holder::class;

    private Serializer $serializer;

    public function twoStatements($data)
    {
        $format = 'json';
        return $this->serializer->deserialize($data, \stdClass::class, $format);
    }

    public function notReturn($data)
    {
        $this->serializer->deserialize($data, \stdClass::class, 'json');
    }

    public function functionCall($data)
    {
        return deserialize($data, \stdClass::class, 'json');
    }

    public function dynamicName($data, $name)
    {
        return $this->serializer->{$name}($data, \stdClass::class, 'json');
    }

    public function otherMethod($data)
    {
        return $this->serializer->serialize($data, 'json', []);
    }

    public function twoArguments($data)
    {
        return $this->serializer->deserialize($data, \stdClass::class);
    }

    public function spread($data, array $rest): \stdClass
    {
        return $this->serializer->deserialize($data, \stdClass::class, ...$rest);
    }

    public function named($data)
    {
        return $this->serializer->deserialize($data, \stdClass::class, format: 'json');
    }

    public function untypedSerializer($serializer, $data)
    {
        return $serializer->deserialize($data, \stdClass::class, 'json');
    }

    public function stringType($data): \App\Holder
    {
        return $this->serializer->deserialize($data, 'App\Holder', 'json');
    }

    public function selfClass($data): \App\Holder
    {
        return $this->serializer->deserialize($data, self::class, 'json');
    }

    public function objectClass($data, Holder $holder)
    {
        return $this->serializer->deserialize($data, $holder::class, 'json');
    }

    public function otherConstant($data): \App\Holder
    {
        return $this->serializer->deserialize($data, Holder::TYPE, 'json');
    }

    public function variableType($data, string $type)
    {
        return $this->serializer->deserialize($data, $type, 'json');
    }

    public function staticClass($data): \App\Holder
    {
        return $this->serializer->deserialize($data, static::class, 'json');
    }

    public function parentClass($data): \App\Base
    {
        return $this->serializer->deserialize($data, parent::class, 'json');
    }

    public function concat($data): \App\Holder
    {
        return $this->serializer->deserialize($data, 'App\\' . 'Holder', 'json');
    }

    public function classConstant($data): \App\Holder
    {
        return $this->serializer->deserialize($data, self::CLASS_TYPE, 'json');
    }

    public function staticConstant($data): \App\Holder
    {
        return $this->serializer->deserialize($data, static::TYPE, 'json');
    }

    public function doubleQuoted($data): \App\Holder
    {
        return $this->serializer->deserialize($data, "App\\Holder", 'json');
    }

    public function intType($data)
    {
        return $this->serializer->deserialize($data, 5, 'json');
    }
}

trait HasParent
{
    private Serializer $serializer;

    public function parentClass($data)
    {
        return $this->serializer->deserialize($data, parent::class, 'json');
    }
}
