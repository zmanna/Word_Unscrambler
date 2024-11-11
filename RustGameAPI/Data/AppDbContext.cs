// Data/AppDbContext.cs
using Microsoft.EntityFrameworkCore;
using RustGameAPI.Models;

namespace RustGameAPI.Data
{
    public class AppDbContext : DbContext
    {
        public AppDbContext(DbContextOptions<AppDbContext> options)
            : base(options)
        {
        }

        public DbSet<User> Users { get; set; }
        public DbSet<Friend> Friends { get; set; }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            // Configure composite primary key for Friend
            modelBuilder.Entity<Friend>()
                .HasKey(f => new { f.UserID, f.FriendUserID });

            // Configure relationships
            modelBuilder.Entity<Friend>()
                .HasOne(f => f.User)
                .WithMany(u => u.Friends)
                .HasForeignKey(f => f.UserID)
                .OnDelete(DeleteBehavior.Restrict);

            modelBuilder.Entity<Friend>()
                .HasOne(f => f.FriendUser)
                .WithMany(u => u.FriendOf)
                .HasForeignKey(f => f.FriendUserID)
                .OnDelete(DeleteBehavior.Restrict);
        }
    }
}
