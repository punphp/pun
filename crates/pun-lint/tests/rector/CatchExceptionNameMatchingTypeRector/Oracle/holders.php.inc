<?php

if (rand()):
    try {
    } catch (Exception $e) {
    }
endif;
switch (rand()) {
    case 1:
        try {
        } catch (Exception $e) {
        }
}
try {
} catch (Error $e) {
}
try {
} catch (A|B) {
    echo $e;
}
-----
<?php

if (rand()):
    try {
    } catch (Exception $e) {
    }
endif;
switch (rand()) {
    case 1:
        try {
        } catch (Exception $e) {
        }
}
try {
} catch (Error $error) {
}
try {
} catch (A|B) {
    echo $error;
}
