use std::{ffi::c_void, mem::MaybeUninit};

use cgmath::{Vector1, Vector2, Vector3, Vector4};

pub trait FromGame {
    fn from_game(data: MaybeUninit<*mut c_void>) -> Self;
}

impl<T: Copy> FromGame for Vector1<T> {
    fn from_game(data: MaybeUninit<*mut c_void>) -> Self {
        unsafe {
            let data = data.assume_init();
            let first_element = data as *mut T;
            Self::new(*first_element)
        }
    }
}

impl<T: Copy> FromGame for Vector2<T> {
    fn from_game(data: MaybeUninit<*mut c_void>) -> Self {
        unsafe {
            let data = data.assume_init();
            let first_element = data as *mut T;
            Self::new(*first_element, *(first_element.offset(1)))
        }
    }
}

impl<T: Copy> FromGame for Vector3<T> {
    fn from_game(data: MaybeUninit<*mut c_void>) -> Self {
        unsafe {
            let data = data.assume_init();
            let first_element = data as *mut T;
            Self::new(
                *first_element,
                *(first_element.offset(1)),
                *(first_element.offset(2)),
            )
        }
    }
}

impl<T: Copy> FromGame for Vector4<T> {
    fn from_game(data: MaybeUninit<*mut c_void>) -> Self {
        unsafe {
            let data = data.assume_init();
            let first_element = data as *mut T;
            Self::new(
                *first_element,
                *(first_element.offset(1)),
                *(first_element.offset(2)),
                *(first_element.offset(3)),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_game_vector() {
        let data = [1, 2, 3, 4];

        let vector1: Vector1<i32> =
            Vector1::from_game(MaybeUninit::new(data.as_ptr() as *mut c_void));
        
        let vector2: Vector2<i32> =
            Vector2::from_game(MaybeUninit::new(data.as_ptr() as *mut c_void));
        
        let vector3: Vector3<i32> =
            Vector3::from_game(MaybeUninit::new(data.as_ptr() as *mut c_void));
        
        let vector4: Vector4<i32> =
            Vector4::from_game(MaybeUninit::new(data.as_ptr() as *mut c_void));

        assert_eq!(vector1.x, 1);
        assert_eq!(vector2.y, 2);
        assert_eq!(vector3.z, 3);
        assert_eq!(vector4.w, 4);
    }
}
