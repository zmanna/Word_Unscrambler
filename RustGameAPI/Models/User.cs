// Models/User.cs
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using Microsoft.AspNetCore.Mvc.ModelBinding;

namespace RustGameAPI.Models
{
    public class User
    {
        [Key]
        [DatabaseGenerated(DatabaseGeneratedOption.Identity)]
        [BindNever]
        public int UserID { get; set; }

        public string Username { get; set; } = string.Empty;
        public string Password { get; set; } = string.Empty;

        // Navigation properties
        public ICollection<Friend> Friends { get; set; } = new List<Friend>();
        public ICollection<Friend> FriendOf { get; set; } = new List<Friend>();
    }
}