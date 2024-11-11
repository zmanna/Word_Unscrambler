// Models/Friend.cs
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace RustGameAPI.Models
{
    public class Friend
    {
        [Key, Column(Order = 0)]
        public int UserID { get; set; }

        [Key, Column(Order = 1)]
        public int FriendUserID { get; set; }

        // Navigation properties
        [ForeignKey("UserID")]
        public User User { get; set; } = null!; // Initialized with null-forgiving operator

        [ForeignKey("FriendUserID")]
        public User FriendUser { get; set; } = null!; // Initialized with null-forgiving operator
    }
}
