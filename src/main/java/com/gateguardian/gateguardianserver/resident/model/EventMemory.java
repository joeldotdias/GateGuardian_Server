package com.gateguardian.gateguardianserver.resident.model;

import jakarta.persistence.*;

import java.time.LocalDateTime;

@Entity
@Table(name = "event_memories")
public class EventMemory {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   @Column(name = "memory_id")
   private Integer memoryId;

   @Column(name = "author_email")
   private String authorEmail;

   @Column(name = "title")
   private String title;

   @Column(name = "body")
   private String body;

   @Column(name = "cover_url")
   private String coverUrl;

   @Column(name = "posted_at")
   private LocalDateTime postedAt;

   public EventMemory() {}

   public EventMemory(Integer memoryId, String authorEmail, String title, String body, String coverUrl, LocalDateTime postedAt) {
      this.memoryId = memoryId;
      this.authorEmail = authorEmail;
      this.title = title;
      this.body = body;
      this.coverUrl = coverUrl;
      this.postedAt = postedAt;
   }

   public Integer getMemoryId() { return memoryId; }
   public void setMemoryId(Integer memoryId) { this.memoryId = memoryId; }

   public String getAuthorEmail() { return authorEmail; }
   public void setAuthorEmail(String authorEmail) { this.authorEmail = authorEmail; }

   public String getTitle() { return title; }
   public void setTitle(String title) { this.title = title; }

   public String getBody() { return body; }
   public void setBody(String body) { this.body = body; }

   public String getCoverUrl() { return coverUrl; }
   public void setCoverUrl(String coverUrl) { this.coverUrl = coverUrl; }

   public LocalDateTime getPostedAt() { return postedAt; }
   public void setPostedAt(LocalDateTime postedAt) { this.postedAt = postedAt; }
}