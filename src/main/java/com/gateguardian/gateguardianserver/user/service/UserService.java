package com.gateguardian.gateguardianserver.user.service;

import com.gateguardian.gateguardianserver.user.model.User;

import java.util.List;

public interface UserService {

   List<User> getUserByEmail(String email);

   void saveUser(User user);
}