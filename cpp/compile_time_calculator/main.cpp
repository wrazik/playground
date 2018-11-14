constexpr int flag (int);
template<class Tag>
struct writer {
  friend constexpr int flag (Tag) {
    return 0;
  }
};
template<bool B, class Tag = int>
struct dependent_writer : writer<Tag> { };
template<
  bool B = noexcept (flag (0)),
  int    =   sizeof (dependent_writer<B>)
>
constexpr int f () {
  return B;
}
int main () {
  constexpr int a = f ();
  constexpr int b = f ();

  static_assert (a != b, "fail");
}
