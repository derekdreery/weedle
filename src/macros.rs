macro_rules! named {
    ($name:ident -> $o:ty, $submac:ident!( $($args:tt)* )) => {
        fn $name(input: $crate::nom::types::CompleteStr) -> $crate::nom::IResult<$crate::nom::types::CompleteStr, $o> {
            $submac!(input, $($args)*)
        }
    };
}

#[macro_export]
macro_rules! weedle {
    ($i:expr, $t:ty) => {
        <$t as $crate::Parse>::parse($i)
    };
}

// Workaround to use `CompleteStr`
macro_rules! re_capture_static (
  ($i:expr, $re:expr) => (
    {
      use $crate::nom::{Err,ErrorKind,IResult};
      use $crate::nom::Slice;

      regex!(RE, $re);
      if let Some(c) = RE.captures(&$i) {
        let v:Vec<_> = c.iter().filter(|el| el.is_some()).map(|el| el.unwrap()).map(|m| $i.slice(m.start()..m.end())).collect();
        let offset = {
          let end = v.last().unwrap();
          end.as_ptr() as usize + end.len() - $i.as_ptr() as usize
        };
        Ok(($i.slice(offset..), v))
      } else {
        let res: IResult<_,_> = Err(Err::Error(error_position!($i, ErrorKind::RegexpCapture::<u32>)));
        res
      }
    }
  )
);
