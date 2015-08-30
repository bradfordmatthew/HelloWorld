
const char* vowls[] = { "a", "A", "e", "E", "i", "I", "o", "O", "u", "U" };

//Maintaining consistency between these and the enum.h file manually a must. TODO: Make this better...
//Junk, Bad, Normal, Good, Best
const char* geteItemDamageType[] = { "Blunt", "Edge", "Pierce", "Arcane" };
const char* geteAttributes[] = { "Intelligence", "Strength", "Dexterity", "Wisdom", "Luck" };
const char* getItemModificationNames[] = { "An Ancient" };
const char* getJunkItemModificationNames[] = { "Rusty", "Broken", "Moldy", "Crusted", "Cracked", "Weak", "Rickety" };
const char* getBadItemModificationNames[] = { "Dented", "Rust Pocked", "Flimsy", "Poor", "Substandard", "Inferior", "Faulty", "Shoddy", "Inadequate", "Third-Rate" };
const char* getNormalItemModificationNames[] = { "Standard", "Usual", "Ordinary", "Conventional", "Normal", "Plane" };
const char* getGoodItemModificationNames[] = {"Superior", "Quality", "Good", "Suitable", "Sturdy", "Sufficient"};
const char* getBestModificationNames[] = { "Fantastic", "Wondrous", "Legendary", "Mythical", "Godly", "Amazing", "Astonishing", "Extraordinary", "Marvelous", "Striking", "Astounding" };

//Basic Game Objects - Will do much re-thinking of this!
const char* getBasicGameObjects[] = { "Chair", "Table", "Desk", "Spoon", "Fork", "Plate", "Door", "House", "Rock", "Tree", "Grass" };

//Materials all items
const char* getJunkItemMaterials[] = { "Rock", "Stone", "Wood" };
const char* getBadItemMaterials[] = { "Tin", "Soft Wood" };
const char* getNormalItemMaterials[] = { "Copper", "Iron", "Glass", "Obsidian", "Hard Wood" };
const char* getGoodItemMaterials[] = { "Steel", "Titanium", "Orchalium", "Gold", "Silver" };
const char* getBestItemMaterials[] = { "Mythril", "Diamond", "Adamantine", "Damascus", "Quick-Silver", "Dragon Bone"};

//Weapons by base damage type
const char* getPierceHandWeapons[] = { "Dagger", "Stiletto", "Knife" };
const char* getPierceLongWeapons[] = { "Spear", "Lance", "Pike", "Pitchfork", "Trident", "Boar Spear" };
const char* getEdge1hWeapons[] = {"Short Sword", "Long Sword", "Cutlass", "Scimitar", "Khopesh", "Falchion", "Dao", "Dha", "Katana"};
const char* getEdge2hWeapons[] = { "Claymore", "Great Sword", "Executioner's Sword", "Boar Sword" };
const char* get1hBluntWeapons[] = {"Mace", "Morning Star", "Axe", "Battle-Axe", "Mattock", "Pickaxe", "War Hammer", "Bardiche", "Hatchet", "Ono", "Tomahawk", "Club", "Rock", "Flail", "Hammer", "Frying Pan"};
const char* get2hBluntWeapons[] = {"Halbred", "Glaive", "Danish Axe", "Lochaber Axe", "Scythe", "Pollaxe", "Voulge"};