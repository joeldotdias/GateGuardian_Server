package com.gateguardian.gateguardianserver.user.service;

import com.gateguardian.gateguardianserver.user.model.User;

import java.util.List;

public interface UserService {

   User getUserByEmail(String email);

   List<User> getUsersBySociety(String society);

   void saveUser(User user);
}