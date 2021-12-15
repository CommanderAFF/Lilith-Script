pub struct Weapons
{
 assault_rifle: String,
 assault_rifle_time: String,
 lr_300: String,
 lr_300_time: String,
 semi_auto_rifle: String,
 semi_auto_rifle_time: String,
 m39_rifle: String,
 m39_rifle_time: String,
 m249: String
 m249_time: String,
 mp5: String,
 mp5_time: String,
 thompson: String,
 thompson_string: String,
 custom: String,
 custom_time: String,
 python: String,
 python_time: String
}


match Menu.weapon{
    "Assault Rifle" => return(AssaultRifle, AssaultRifleTime),
    "M249" => return(M249, M249Time),
    _ => return(defaultnothing, 0),
  };