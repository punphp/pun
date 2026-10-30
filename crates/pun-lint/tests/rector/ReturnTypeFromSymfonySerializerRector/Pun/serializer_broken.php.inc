<?php

namespace App;

use Symfony\Component\Serializer\Serializer;

final class Holder
{
    private Serializer $serializer;

    public function leadingSlash($data)
    {
        return $this->serializer->deserialize($data, '\App\Holder', 'json');
    }

    public function upperCaseClass($data)
    {
        return $this->serializer->deserialize($data, Holder::CLASS, 'json');
    }

    public function dynamicConstant($data, string $c)
    {
        return $this->serializer->deserialize($data, Holder::{$c}, 'json');
    }
}

$anonymous = new class {
    private Serializer $serializer;

    public function selfClass($data)
    {
        return $this->serializer->deserialize($data, self::class, 'json');
    }
};
-----
<?php

namespace App;

use Symfony\Component\Serializer\Serializer;

final class Holder
{
    private Serializer $serializer;

    public function leadingSlash($data): \App\Holder
    {
        return $this->serializer->deserialize($data, '\App\Holder', 'json');
    }

    public function upperCaseClass($data): \App\Holder
    {
        return $this->serializer->deserialize($data, Holder::CLASS, 'json');
    }

    public function dynamicConstant($data, string $c)
    {
        return $this->serializer->deserialize($data, Holder::{$c}, 'json');
    }
}

$anonymous = new class {
    private Serializer $serializer;

    public function selfClass($data)
    {
        return $this->serializer->deserialize($data, self::class, 'json');
    }
};
