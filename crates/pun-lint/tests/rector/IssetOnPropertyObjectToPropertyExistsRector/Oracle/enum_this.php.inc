<?php

namespace Oracle\PropertyExistsEnum;

enum Status: string
{
    case On = 'on';

    public function check()
    {
        return isset($this->value) && isset($this->name) && !isset($this->other);
    }
}
-----
<?php

namespace Oracle\PropertyExistsEnum;

enum Status: string
{
    case On = 'on';

    public function check()
    {
        return $this->value !== null && $this->name !== null && (!property_exists($this, 'other') || $this->other === null);
    }
}
